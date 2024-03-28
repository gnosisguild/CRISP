import React from 'react';
import Logo from '../assets/icons/logo.svg';

const Navbar: React.FC = () => {
  return (
    <nav className="bg-white-900 shadow-md border-b-2 border-twilight-blue-200 ">
      <div className="max-w-7xl mx-auto px-4 sm:px-2 lg:px-9">
        <div className="flex justify-between h-20 items-center">
          {/* Logo */}
          <img src={Logo} alt="CRISP Logo" className="h-8" />

          {/* Links de navegación */}
          <div className="hidden sm:flex sm:items-center sm:gap-8">
            <a href="#about" className="text-twilight-blue-900 font-bold hover:text-twilight-blue-600">About</a>
            <a href="#daily-polls" className="text-twilight-blue-900 font-bold hover:text-twilight-blue-600">Daily Polls</a>
            <a href="#historic-polls" className="text-twilight-blue-900 font-bold hover:text-twilight-blue-600">Historic Polls</a>
          </div>

          {/* Botones de acción */}
          <div className="flex items-center gap-4">
            <a href="#login" className="text-twilight-blue-900 font-bold hover:text-twilight-blue-600">Login</a>
            <button className="button">
              Register
            </button>
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
